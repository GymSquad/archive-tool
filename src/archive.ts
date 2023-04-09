import { spawn } from "child_process";
import { rename } from "fs/promises";
import { join } from "path";
import { PWYB_COLLECTIONS_PATH } from "./config";

function fillZero(num: number) {
  return `0${num}`.slice(-2);
}

export function getNewCollectionName(date: Date) {
  const collectionName = `${date.getFullYear()}${fillZero(
    date.getMonth() + 1
  )}${fillZero(date.getDate())}`;
  return collectionName;
}

export function createCollection(name: string) {
  return new Promise<string>((resolve, reject) => {
    const createCollectionProcess = spawn("wb-manager", ["init", name]);

    createCollectionProcess.on("close", (code: number) => {
      if (code !== 0) {
        return reject("failed to create collection");
      }
      resolve(name);
    });
  });
}

function getWarcFileName(url: string) {
  return new URL(url).hostname;
}

export function archiveUrl(url: string) {
  return new Promise<string>((resolve) => {
    const warcFile = getWarcFileName(url);

    const archiveProcess = spawn("wget", [
      "--mirror",
      "--warc-file",
      warcFile,
      "--user-agent",
      "Mozilla",
      "--no-verbose",
      url,
    ]);

    archiveProcess.stderr.setEncoding("utf-8").on("data", (data: string) => {
      const match = data.match(/"(.*)"/);
      if (match) {
        console.log("downloading", match[1]);
      }
    });

    archiveProcess.on("close", () => {
      console.log("done");
      resolve(warcFile);
    });
  });
}

export async function addToCollection(
  collectionName: string,
  warcFile: string
) {
  const archivePath = join(PWYB_COLLECTIONS_PATH, collectionName, "archive");
  const filename = `${warcFile}.warc.gz`;
  const newPath = join(archivePath, filename);
  await rename(filename, newPath);
}
