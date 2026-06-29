import { AssetImage } from "./AssetImage";

interface Props {
  frontKey: string;
  backKey: string;
  /** When false the card shows its back (face down); flipping to true reveals the face. */
  faceUp: boolean;
  title?: string;
}

/**
 * The card-flip reveal (FR-010). Presentation-only juice: it animates the *canonical* front
 * and back images the core produced — it never changes what the card is. The 3D flip is the
 * surprise moment of the deck-creation finale.
 */
export function CardFlip({ frontKey, backKey, faceUp, title }: Props) {
  return (
    <div className="card-flip" data-testid="sample-card" data-faceup={faceUp}>
      <div className={`card-flip__inner${faceUp ? " is-faceup" : ""}`}>
        <div className="card-flip__face card-flip__back" aria-hidden={faceUp}>
          <AssetImage assetKey={backKey} alt="card back" className="card-flip__img" />
        </div>
        <div className="card-flip__face card-flip__front" aria-hidden={!faceUp}>
          <AssetImage assetKey={frontKey} alt={title ?? "card front"} className="card-flip__img" />
        </div>
      </div>
    </div>
  );
}
