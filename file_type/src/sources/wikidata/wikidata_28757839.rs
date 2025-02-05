use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757839: FileFormat = FileFormat {
    id: 28_757_839,
    source_type: SourceType::Wikidata,
    name: "Genecyst Backup RAM",
    extensions: &["gsv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
