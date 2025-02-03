use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979270: FileFormat = FileFormat {
    id: 27_979_270,
    source_type: SourceType::Wikidata,
    name: "TheDraw Save File",
    extensions: &["td"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
