use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66146236: FileFormat = FileFormat {
    id: 66_146_236,
    source_type: SourceType::Wikidata,
    name: "InfoPath Form Definition File",
    extensions: &["xsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
