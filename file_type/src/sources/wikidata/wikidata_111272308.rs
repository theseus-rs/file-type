use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272308: FileFormat = FileFormat {
    id: 111_272_308,
    source_type: SourceType::Wikidata,
    name: "Ensoniq KT instrument file",
    extensions: &["efk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
