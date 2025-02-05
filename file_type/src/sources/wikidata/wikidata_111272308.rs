use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111272308: FileFormat = FileFormat {
    id: 111_272_308,
    source_type: SourceType::Wikidata,
    name: "Ensoniq KT instrument file",
    extensions: &["efk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
