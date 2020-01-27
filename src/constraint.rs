enum Subject {
    Everyone,
    User(userid),
}

struct Constraint {
    subject: Subject,
}