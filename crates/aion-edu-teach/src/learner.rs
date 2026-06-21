//! The learner channel. The professor reaches the learner only through this.
//!
//! For an autonomous run the learner is simulated by a cheaper model holding a
//! correct mental model of the material; a human learner is a later backend.

use serde_json::{json, Value};

use crate::client::{text_of, Client};
use crate::error::Result;

const STUDENT_MODEL: &str = "claude-haiku-4-5";

/// Anything the professor can ask and get an answer from.
pub trait Learner {
    fn reply(&mut self, prof_message: &str) -> Result<String>;
}

/// A simulated student: a cheaper model briefed with the answer key.
pub struct SimulatedLearner<'a> {
    client: &'a Client,
    system: String,
    history: Vec<Value>,
}

impl<'a> SimulatedLearner<'a> {
    pub fn new(client: &'a Client, learner: &str, lesson_id: &str) -> Self {
        let system = format!(
            "You are {learner}, a sharp, cooperative student. Answer concisely and \
             correctly; do the work yourself when asked. Context for this lesson: {}",
            student_brief(lesson_id)
        );
        Self { client, system, history: Vec::new() }
    }
}

impl Learner for SimulatedLearner<'_> {
    fn reply(&mut self, prof_message: &str) -> Result<String> {
        self.history.push(json!({"role": "user", "content": prof_message}));
        let resp = self.client.message(
            STUDENT_MODEL,
            &self.system,
            Value::Array(self.history.clone()),
            None,
            900,
        )?;
        let text = text_of(&resp);
        self.history.push(json!({"role": "assistant", "content": text}));
        Ok(text)
    }
}

/// Per-lesson correct mental model handed to the simulated student (not the prof).
fn student_brief(lesson_id: &str) -> &'static str {
    match lesson_id {
        "cs340-u1-l1" =>
            "Lamport clocks. A send/local event does clock+=1. On receive of a message with \
             timestamp ts, the rule is clock = max(local, ts) + 1. If asked to write or fix \
             process.py, output the full Process class where recv does exactly \
             self.clock = max(self.clock, ts) + 1; return self.clock. The converse fails: \
             C(a) < C(b) does NOT imply a happened-before b (they can be concurrent), because \
             total-ordering integers cannot encode independent process histories.",
        "cs340-u1-l2" =>
            "Vector clocks. Each process keeps a vector v of length n. tick/send: v[self]+=1. \
             recv(msg): MERGE componentwise v = [max(v[i], msg[i]) for i] THEN v[self]+=1. \
             happens_before(a,b): all(a[i]<=b[i]) and a!=b. concurrent(a,b): neither happens_before. \
             Space cost is O(n) and fundamental — each process history is tracked independently. \
             If asked to write process.py, output the correct recv (merge then bump) and happens_before.",
        "math110-u1-l1" =>
            "Column picture. columns(A) returns the COLUMNS (transpose). Ax = x0*col0 + x1*col1; \
             build matvec from columns(), not row dot products. solvable_2x2(A,b): det=ad-bc; if \
             det!=0 always solvable; if det==0 (dependent columns) b is solvable only if on the line \
             of a nonzero column, i.e. col[0]*b[1]-col[1]*b[0]==0. Output correct columns/matvec/solvable_2x2.",
        "math110-u1-l2" =>
            "Projection onto a line. project(a,b) = (dot(a,b)/dot(a,a)) * a, i.e. s=dot(a,b)/dot(a,a); \
             return [s*ai for ai in a]. The error e=b-p is orthogonal to a (a dot e = 0), and that \
             orthogonality makes p the closest point (1-D least squares). Output correct project.",
        "math110-u1-l3" =>
            "Least squares line y=c+d*x via the normal equations. n=len, Sx=sum x, Sy=sum y, \
             Sxx=sum x*x, Sxy=sum x*y, det=n*Sxx-Sx*Sx; c=(Sy*Sxx-Sx*Sxy)/det; d=(n*Sxy-Sx*Sy)/det. \
             Residual orthogonal to both columns (sum r=0 and sum x*r=0). Derive the equations, no \
             library solver. Output correct fit_line.",
        "math110-u2-l1" =>
            "Eigenvalues of a 2x2. Av=λv (A scales eigenvector v by λ). Eigenvalues solve \
             λ²-(trace)λ+det=0: t=A[0][0]+A[1][1], d=A[0][0]*A[1][1]-A[0][1]*A[1][0], disc=t*t-4*d; \
             return ((t+sqrt(disc))/2, (t-sqrt(disc))/2). For [[2,1],[1,2]] the eigenvalues are 1 and 3, \
             with v=[1,1] for λ=3. Output correct eigenvalues_2x2.",
        "phys101-u1-l1" =>
            "Pendulum from dimensions. Only sqrt(L/g) has units of time, so T=2*pi*sqrt(L/g); mass \
             cannot appear. predict_ratio(L1,g1,L2,g2)=sqrt((L1/g1)/(L2/g2)). simulate_period \
             integrates theta'' = -(g/L)*sin(theta) with symplectic Euler (update omega then theta), \
             dt=1e-4; record zero crossings; period = crossings[2]-crossings[0]. Output correct \
             period/predict_ratio/simulate_period.",
        "phys101-u1-l2" =>
            "Energy of a mass on a spring. E=0.5*m*v*v+0.5*k*x*x. Use SYMPLECTIC Euler: update velocity \
             FIRST (v_new=v-(k/m)*x*dt) then position with the new velocity (x_new=x+v_new*dt). Explicit \
             Euler leaks energy. max_speed(omega,A)=omega*A. Output correct energy/step/simulate/max_speed.",
        "phys101-u1-l3" =>
            "Simple harmonic motion. x(t)=A*cos(omega*t+phi) — COSINE with phase, not sin. \
             omega_spring(k,m)=sqrt(k/m); omega_pendulum(g,L)=sqrt(g/L); period(omega)=2*pi/omega recovers \
             2*pi*sqrt(L/g). Output correct omega_spring/omega_pendulum/shm_x/period.",
        "math150-u1-l1" =>
            "Mutilated chessboard. Color each cell by (r+c)%2. A domino covers one even + one odd cell, \
             so (#even - #odd) is invariant. imbalance(cells): e=sum(1 for r,c in cells if (r+c)%2==0); \
             return abs(2*e-len(cells)). tileable_parity(cells)=(len%2==0 and imbalance==0). 8x8 minus two \
             opposite corners has imbalance 2. Parity is necessary, not sufficient. Output correct \
             imbalance/tileable_parity.",
        "math150-u1-l2" =>
            "Parity invariant. Flipping two coins changes the head count by an even amount, so head-count \
             parity is invariant. heads(s)=sum(s); same_parity(a,b)=(heads(a)%2==heads(b)%2); \
             reachable(start,target)=(len(start)==len(target) and same_parity(start,target)). Output \
             correct same_parity/reachable.",
        "math150-u1-l3" =>
            "Nim and the XOR invariant. nim_sum(piles) = XOR of all pile sizes (reduce a^b from 0). \
             is_winning = nim_sum != 0. winning_move(piles): s=nim_sum(piles); if s==0 return None; for \
             i,p in enumerate(piles): t=p^s; if t<p: return (i,t). Output correct nim_sum/is_winning/winning_move.",
        "cs220-u1-l1" =>
            "Binary search on a SORTED array, half-open interval [lo,hi): lo=0,hi=len(a); while lo<hi: \
             mid=(lo+hi)//2; if a[mid]<x: lo=mid+1 else: hi=mid; return lo if lo<len(a) and a[lo]==x \
             else -1. Invariant: if x is present it is in a[lo:hi]; the interval strictly shrinks; about \
             log2(n) iterations so O(log n). The classic bug is a closed interval with hi=len(a) and <= \
             that indexes out of bounds. Output the correct search.",
        "cs250-u1-l1" =>
            "Integer square root. isqrt(n) = largest a with a*a <= n. Post-condition a*a <= n < (a+1)**2. \
             Derive the loop from it: a=0; while (a+1)*(a+1) <= n: a+=1; return a. Invariant a*a <= n; \
             a strictly increases and is bounded so it terminates. The classic bug `while a*a <= n` \
             overshoots by one. Output the correct isqrt.",
        "ee210-u1-l1" =>
            "Hamming(7,4). Positions 1..7 are [p1,p2,d1,p4,d2,d3,d4]. encode: p1=d1^d2^d4, p2=d1^d3^d4, \
             p4=d2^d3^d4. syndrome(code): unpack p1,p2,d1,p4,d2,d3,d4; s1=p1^d1^d2^d4, s2=p2^d1^d3^d4, \
             s4=p4^d2^d3^d4; return s1 + 2*s2 + 4*s4 — that integer is the 1-based position of the \
             flipped bit, 0 if clean. decode flips position s (if s!=0) then returns bits at positions \
             3,5,6,7. Output the correct syndrome.",
        "astro101-u1-l1" =>
            "Kepler's third law, units G=1. Circular orbit radius r about mass M: v=sqrt(M/r), \
             T=2*pi*r/v. T^2/r^3 = 4*pi^2/M — the SAME constant for every orbit. circular_period(M,r) = \
             2*pi*r/circular_speed(M,r). period_ratio(M,r1,r2) = (r1/r2)**1.5; quadrupling r multiplies \
             T by 4**1.5=8. Output correct circular_period and period_ratio.",
        "cs270-u1-l1" =>
            "Turing machine. Config (tape: dict pos->symbol, head: int, state: str); BLANK='_'. \
             step(tape,head,state,table): sym=tape.get(head,BLANK); if (state,sym) not in table return \
             (tape,head,'HALT'); (new_sym,move,new_state)=table[(state,sym)]; tape[head]=new_sym; \
             head+=move; return (tape,head,new_state). run loops step until state=='HALT'. With the \
             given INCREMENT_TABLE this turns '1011' into '1100' and '111' into '1000' (carry extends \
             the tape left). Output the correct step (WRITE the symbol AND move the head).",
        "math210-u1-l1" =>
            "Eulerian paths. degrees(n,edges): d[u]+=1,d[v]+=1 per edge. odd_degree_count = number of \
             vertices with odd degree = sum(1 for x in degrees(n,edges) if x%2==1). \
             has_eulerian_circuit = (odd_degree_count==0). has_eulerian_path = (odd_degree_count in (0,2)). \
             Königsberg: four landmasses all odd degree -> not 0 or 2 -> no Eulerian walk. Output correct \
             odd_degree_count and has_eulerian_path.",
        "bio101-u1-l1" =>
            "Population genetics. Alleles A(p), a(q=1-p). Hardy-Weinberg: hardy_weinberg(p)=(p*p, 2*p*q, \
             q*q) — sums to 1; DON'T forget the 2 on the heterozygote. mean_fitness uses those. \
             next_p(p,w) with w=(wAA,wAa,waa): wbar=mean_fitness(p,w); return (p*p*wAA + p*q*wAa)/wbar \
             (include the heterozygote term). Selection against recessive (waa=0) raises p. Output correct \
             hardy_weinberg and next_p.",
        "gt101-u1-l1" =>
            "Zero-sum 2x2 games. A = row player's payoffs. maximin = max over rows of min(row): \
             max(min(r) for r in A). minimax = min over cols of max(col): cols=list(zip(*A)); \
             min(max(c) for c in cols). Saddle point iff maximin==minimax (the pure value). For 2x2 with \
             no saddle, value=(a00*a11 - a01*a10)/(a00+a11-a01-a10). Output correct maximin (min of each \
             row), minimax (max of each col), and game_value.",
        "it201-u1-l1" =>
            "Shannon entropy in BITS: H(p) = -sum(pi*log2(pi)) over pi>0 (zero-prob terms contribute 0). \
             entropy(p) = -sum(pi*math.log2(pi) for pi in p if pi>0). Fair coin [0.5,0.5]->1 bit; certain \
             [1,0]->0; uniform over n -> log2(n) (4 outcomes->2, 8->3). The classic bug uses natural log \
             or drops the minus sign. Output correct entropy.",
        "math230-u1-l1" =>
            "Extended Euclidean algorithm. gcd(a,b)=gcd(b,a%b) until b=0 (given). ext_gcd(a,b): if b==0 \
             return (a,1,0); g,x1,y1=ext_gcd(b,a%b); return (g, y1, x1-(a//b)*y1) — satisfies a*x+b*y=g \
             (Bezout). The classic bug returns (g,x1,y1) without recombining. mod_inverse(a,m): \
             g,x,_=ext_gcd(a%m,m); if g!=1 return None; return x%m. Output correct ext_gcd and mod_inverse.",
        "math310-u1-l1" =>
            "Group axioms over elems with op(a,b). identity(elems,op): return the e (if any) with \
             op(e,a)==a and op(a,e)==a for ALL a (SEARCH, don't assume elems[0]); else None. \
             has_inverses(elems,op,e): all(any(op(a,b)==e and op(b,a)==e for b in elems) for a in elems). \
             is_group = closed and associative and identity is not None and has_inverses. Z_n under + is a \
             group (identity 0); {0,1,2,3} under * mod 4 is not (0 has no inverse). Output correct identity \
             and has_inverses.",
        "logic101-u1-l1" =>
            "Boolean formulas as f(env)->bool; all_envs(vars) yields every assignment (given). \
             is_tautology(f,vars) = all(f(env) for env in all_envs(vars)). is_satisfiable(f,vars) = \
             any(f(env) for env in all_envs(vars)). is_contradiction = not is_satisfiable. The classic bug \
             returns f of only the FIRST env (a `return` inside the loop instead of all/any). Output \
             correct is_tautology and is_satisfiable.",
        "sys301-u1-l1" =>
            "Consistent hashing on a ring. h(s) hashes to [0,RING). build_ring(nodes)=sorted (h(n),n). \
             node_for(key,ring): k=h(key); for (pos,node) in ring (sorted ascending): if pos>=k return \
             node; if none matched return ring[0][1] (wrap clockwise). The stub returns ring[0][1] \
             always. Key property: removing a node only moves the keys it owned (~1/N). Output the \
             correct node_for.",
        "sys310-u1-l1" =>
            "Quorum consensus. is_strongly_consistent(n,r,w) = (r+w>n) — STRICTLY greater; r+w==n does \
             NOT guarantee overlap. read_quorum(replicas,r): replicas are (version,value); take \
             replicas[:r] and return the value with the highest version: max(replicas[:r], key=lambda \
             rv: rv[0])[1]. The stub uses >= and returns the first replica ignoring versions. Output \
             correct is_strongly_consistent and read_quorum.",
        "sys320-u1-l1" =>
            "Token-bucket rate limiter. allow(now): elapsed=now-self.last; self.last=now; \
             self.tokens=min(self.capacity, self.tokens + self.rate*elapsed); if self.tokens>=1: \
             self.tokens-=1; return True; else return False. Capacity 3, rate 1: three immediate \
             allows then deny; after 2s, two more; refill capped at capacity. The stub returns True \
             always. Output the correct allow.",
        "sys330-u1-l1" =>
            "Circuit breaker, states CLOSED/OPEN/HALF_OPEN. allow(now): if state==OPEN: if \
             now-self.opened_at>=self.cooldown: state=HALF_OPEN; return True (one trial); else return \
             False; otherwise (CLOSED or HALF_OPEN) return True. on_result(success,now): if success: \
             state=CLOSED, failures=0; else if state==HALF_OPEN: state=OPEN, opened_at=now; else \
             failures+=1 and if failures>=threshold: state=OPEN, opened_at=now. The stub allows \
             everything and records nothing. Output correct allow and on_result.",
        "cs330-u1-l1" =>
            "Rational-number ADT. Rep invariant: den>0 and gcd(abs(num),den)==1 (lowest terms, sign in \
             numerator). Abstraction function AF(num,den)=num/den. make(num,den): if den==0 raise \
             ValueError; if den<0 set num,den=-num,-den; g=gcd(abs(num),den); return (num//g, den//g). \
             add(a,b): n=numer(a)*denom(b)+numer(b)*denom(a); d=denom(a)*denom(b); return make(n,d) \
             (must REDUCE). check_rep(r): n,d=r; return d>0 and gcd(abs(n),d)==1. The stub stores \
             fractions unreduced and check_rep returns True always. Output correct make, add, check_rep.",
        "cs440-u1-l1" =>
            "Byzantine agreement. agreement_possible(n,f) = (n >= 3*f + 1) — the Byzantine bound (3 \
             generals + 1 traitor impossible, 4 ok); the stub wrongly uses 2*f+1. om_rounds(f) = f + 1 \
             (Oral Messages OM(f)). majority(values, default): from collections import Counter; \
             val,cnt = Counter(values).most_common(1)[0]; return val if cnt*2 > len(values) else \
             default (STRICT majority; a tie or plurality returns default). With n>=3f+1 the n-f loyal \
             nodes outvote the f traitors. Output correct agreement_possible, om_rounds, majority.",
        "cs450-u1-l1" =>
            "Linearizability. Ops are tuples (kind,value,start,end); KIND,VALUE,START,END=0,1,2,3. \
             precedes(a,b) = a[END] < b[START] (a finishes strictly before b starts — real-time order). \
             legal_register(order, initial=0): cur=initial; for op in order: if op[KIND]=='W': \
             cur=op[VALUE]; else (read): if op[VALUE]!=cur: return False; return True. The stub compares \
             start times and returns True always. A history is linearizable iff some real-time-respecting \
             permutation is legal. Output correct precedes and legal_register.",
        _ => "Answer correctly and do the work the professor asks.",
    }
}
