use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111263105: FileFormat = FileFormat {
    id: 111_263_105,
    source_type: SourceType::Wikidata,
    name: "Typhoon wave file",
    extensions: &["c01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
