use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272521: FileFormat = FileFormat {
    id: 111_272_521,
    source_type: SourceType::Wikidata,
    name: "Ensoniq VFX-SD instrument file",
    extensions: &["efv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
