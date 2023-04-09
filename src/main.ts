import { PrismaClient } from "@prisma/client";
import rimraf from "rimraf";
import { addToCollection, archiveUrl, getNewCollectionName } from "./archive";
import { checkUrl } from "./checkUrl";
import { getAllUrls, updateUrlValidity } from "./db";

const prisma = new PrismaClient();

async function main() {
  const collectionName = getNewCollectionName(new Date());

  const urls = await getAllUrls(prisma);

  urls.forEach(async ({ id, url }) => {
    const isValid = await checkUrl(url);

    if (!isValid) {
      return await updateUrlValidity(prisma, id, isValid);
    }
    const archived = await archiveUrl(url);

    // remove the html, css, js files get by wget
    // the directory name is the same as the warc file name
    await rimraf(archived);

    await addToCollection(collectionName, archived);
  });
}

main()
  .then(async () => {
    await prisma.$disconnect();
  })
  .catch(async (err) => {
    console.error(err);
    await prisma.$disconnect();
    process.exit(1);
  });
