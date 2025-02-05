use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61963304: FileFormat = FileFormat {
    id: 61_963_304,
    source_type: SourceType::Wikidata,
    name: "Microsoft Front Page Binary Tree Index",
    extensions: &["btr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
