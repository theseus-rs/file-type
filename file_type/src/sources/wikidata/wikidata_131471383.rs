use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131471383: FileFormat = FileFormat {
    id: 131_471_383,
    source_type: SourceType::Wikidata,
    name: "Compressed MGH file format",
    extensions: &["mgh.gz", "mgz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
