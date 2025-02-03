use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009460: FileFormat = FileFormat {
    id: 111_009_460,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Calendar File format",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
