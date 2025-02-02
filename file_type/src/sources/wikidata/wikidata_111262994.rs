use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111262994: FileFormat = FileFormat {
    id: 111_262_994,
    source_type: SourceType::Wikidata,
    name: "Aureal 'Aspen' bank file",
    extensions: &["arl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
