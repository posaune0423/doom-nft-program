import assert from "node:assert/strict";
import { describe, test } from "node:test";

import { assertUrlReachable, decodeAssetUri } from "./mint";

describe("decodeAssetUri", () => {
  test("returns the URI stored in the on-chain asset data", () => {
    const accountData = Buffer.concat([
      Buffer.from([1]),
      Buffer.alloc(32, 7),
      Buffer.from([2]),
      Buffer.alloc(32, 8),
      encodeBorshString("DOOM INDEX #1"),
      encodeBorshString("https://example.com/base/1.json"),
      Buffer.from([0]),
      Buffer.from([9, 9, 9]),
    ]);

    assert.equal(decodeAssetUri(accountData), "https://example.com/base/1.json");
  });
});

describe("assertUrlReachable", () => {
  test("falls back to GET when HEAD is not supported", async () => {
    const calls: Array<{ url: string; method: string }> = [];
    await assertUrlReachable("https://example.com/asset.png", "Image", async (url, init) => {
      calls.push({ url: String(url), method: init?.method ?? "GET" });
      if (init?.method === "HEAD") {
        return new Response(null, { status: 405, statusText: "Method Not Allowed" });
      }

      return new Response("ok", { status: 200, statusText: "OK" });
    });

    assert.deepEqual(calls, [
      { url: "https://example.com/asset.png", method: "HEAD" },
      { url: "https://example.com/asset.png", method: "GET" },
    ]);
  });

  test("throws when both HEAD and GET fail", async () => {
    await assert.rejects(
      () =>
        assertUrlReachable("https://example.com/asset.png", "Image", async (_url, init) => {
          if (init?.method === "HEAD") {
            return new Response(null, { status: 403, statusText: "Forbidden" });
          }

          return new Response(null, { status: 404, statusText: "Not Found" });
        }),
      /Image fetch failed: 404 Not Found/,
    );
  });
});

function encodeBorshString(value: string): Buffer {
  const text = Buffer.from(value, "utf8");
  const length = Buffer.alloc(4);
  length.writeUInt32LE(text.length, 0);
  return Buffer.concat([length, text]);
}
