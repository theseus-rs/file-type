use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122463531: FileFormat = FileFormat {
    id: 122_463_531,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual Basic Include file",
    extensions: &["bi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
