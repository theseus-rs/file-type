use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116850774: FileFormat = FileFormat {
    id: 116_850_774,
    source_type: SourceType::Wikidata,
    name: "Card File",
    extensions: &["dtp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
