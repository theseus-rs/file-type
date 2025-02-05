use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59608150: FileFormat = FileFormat {
    id: 59_608_150,
    source_type: SourceType::Wikidata,
    name: "Microsoft Expression Media",
    extensions: &["ivc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
