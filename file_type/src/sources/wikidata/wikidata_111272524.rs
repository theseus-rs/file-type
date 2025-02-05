use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272524: FileFormat = FileFormat {
    id: 111_272_524,
    source_type: SourceType::Wikidata,
    name: "Ensoniq instrument file",
    extensions: &["efx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
