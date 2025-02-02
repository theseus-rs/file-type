use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009741: FileFormat = FileFormat {
    id: 111_009_741,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Fax Cover File format",
    extensions: &["fax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
