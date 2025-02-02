use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757839: FileFormat = FileFormat {
    id: 28_757_839,
    source_type: SourceType::Wikidata,
    name: "Genecyst Backup RAM",
    extensions: &["gsv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
