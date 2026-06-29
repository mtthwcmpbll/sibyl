import type { CardStyleGuide } from "../platform";
import { AssetImage } from "./AssetImage";

/** Shows the generated card style guide: chrome, back, flourish, and designated shader areas. */
export function StyleGuidePreview({ guide }: { guide: CardStyleGuide }) {
  return (
    <div className="style-guide" data-testid="style-guide-preview">
      <div className="style-guide__assets">
        <figure>
          <AssetImage assetKey={guide.borderChromeKey} alt="border and chrome" className="sg-asset" />
          <figcaption>Border &amp; chrome</figcaption>
        </figure>
        <figure>
          <AssetImage assetKey={guide.cardBackKey} alt="card back" className="sg-asset" />
          <figcaption>Card back</figcaption>
        </figure>
        {guide.flourishes.map((f, i) => (
          <figure key={f.assetKey}>
            <AssetImage assetKey={f.assetKey} alt={`flourish ${i + 1}`} className="sg-asset" />
            <figcaption>Flourish {i + 1}</figcaption>
          </figure>
        ))}
      </div>
      <dl className="style-guide__meta">
        <dt>Front layout</dt>
        <dd>aspect {guide.cardFrontLayout.aspect}</dd>
        <dt>Animated shader areas</dt>
        <dd data-testid="shader-areas">
          {guide.shaderAreas.map((a) => `${a.id} (${a.kind})`).join(", ") || "none"}
        </dd>
      </dl>
    </div>
  );
}
