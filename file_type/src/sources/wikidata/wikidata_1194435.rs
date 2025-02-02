use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1194435: FileFormat = FileFormat {
    id: 1_194_435,
    source_type: SourceType::Wikidata,
    name: "MPEG-2 transport stream",
    extensions: &["ts", "tsa", "tsv"],
    media_types: &["video/mp2t"],
    internal_signatures: &[],
    related_formats: &[],
};
