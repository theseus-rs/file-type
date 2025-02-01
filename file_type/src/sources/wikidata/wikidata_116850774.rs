use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116850774: FileFormat = FileFormat {
    id: 116_850_774,
    puid: "wikidata/116850774",
    name: "Card File",
    extensions: &["dtp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
