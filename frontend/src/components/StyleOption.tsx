import type { StyleOption as StyleOptionData } from "../platform";
import { AssetImage } from "./AssetImage";

interface Props {
  option: StyleOptionData;
  selected: boolean;
  onSelect: (id: string) => void;
}

/** One candidate suit-icon set the owner can pick (FR-004/005). */
export function StyleOption({ option, selected, onSelect }: Props) {
  return (
    <button
      type="button"
      className={`style-option${selected ? " is-selected" : ""}`}
      aria-pressed={selected}
      data-testid={`style-option-${option.id}`}
      onClick={() => onSelect(option.id)}
    >
      <span className="style-option__label">{option.label}</span>
      <span className="style-option__icons">
        {option.suitIcons.map((icon) => (
          <AssetImage
            key={icon.suit}
            assetKey={icon.imageKey}
            alt={`${icon.suit} icon`}
            className="suit-icon"
          />
        ))}
      </span>
    </button>
  );
}
