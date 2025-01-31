use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119999757: FileFormat = FileFormat {
    id: 119_999_757,
    puid: "wikidata/119999757",
    name: "DJ RingTone File",
    extensions: &["djr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
