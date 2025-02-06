use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272310: FileFormat = FileFormat {
    id: 111_272_310,
    source_type: SourceType::Wikidata,
    name: "Ensoniq SQ1/SQ2/KS32 instrument file",
    extensions: &["efq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
