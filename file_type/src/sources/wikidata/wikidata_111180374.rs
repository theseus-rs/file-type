use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111180374: FileFormat = FileFormat {
    id: 111_180_374,
    source_type: SourceType::Wikidata,
    name: "PressWriter File",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
