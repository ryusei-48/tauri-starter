-- CreateTable
CREATE TABLE "User" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL
);

CREATE TABLE "Tag" (
    "id" INTEGER PRIMARY KEY, "ison" BLOB,
    "update_at" TIMESTAMP NOT NULL DEFAULT (DATETIME('now','localtime')),
    "register_at" TIMESTAMP NOT NULL DEFAULT (DATETIME('now','localtime'))
);

-- CreateIndex
CREATE UNIQUE INDEX "User_email_key" ON "User"("email");