use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111262641: FileFormat = FileFormat {
    id: 111_262_641,
    source_type: SourceType::Wikidata,
    name: "Muon DS404 bank file",
    extensions: &["404"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
