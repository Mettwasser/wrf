export interface RegisterSession {
    sessionId: string;
    expiry: Date;
    verificationCode: string;
}
