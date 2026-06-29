import { useEffect, useState } from "react";

import { getClient } from "../platform";

interface Props {
  assetKey: string;
  alt: string;
  className?: string;
}

/** Resolves a storage key to a displayable image via the active client (Tauri or mock). */
export function AssetImage({ assetKey, alt, className }: Props) {
  const [src, setSrc] = useState<string | undefined>();

  useEffect(() => {
    let live = true;
    getClient()
      .then((c) => c.getAsset(assetKey))
      .then((a) => {
        if (live) setSrc(a.dataUrl);
      })
      .catch(() => {
        /* leave broken-image fallback */
      });
    return () => {
      live = false;
    };
  }, [assetKey]);

  return <img src={src} alt={alt} className={className} data-asset-key={assetKey} />;
}
