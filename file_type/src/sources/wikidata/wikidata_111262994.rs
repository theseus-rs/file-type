use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111262994: FileFormat = FileFormat {
    id: 111_262_994,
    source_type: SourceType::Wikidata,
    name: "Aureal 'Aspen' bank file",
    extensions: &["arl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
