use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009215: FileFormat = FileFormat {
    id: 111_009_215,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Banner File Format",
    extensions: &["ban"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
