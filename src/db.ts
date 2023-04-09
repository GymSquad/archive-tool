import { PrismaClient } from "@prisma/client";

export function getAllUrls(db: PrismaClient) {
  return db.website.findMany({
    select: {
      id: true,
      url: true,
    },
  });
}

export function updateUrlValidity(
  db: PrismaClient,
  id: string,
  isValid: boolean
) {
  return db.website.update({
    where: { id },
    data: { isValid },
  });
}
