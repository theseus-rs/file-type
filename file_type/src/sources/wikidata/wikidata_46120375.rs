use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_46120375: FileFormat = FileFormat {
    id: 46_120_375,
    source_type: SourceType::Wikidata,
    name: "Lotus Notes Database file format, version 4",
    extensions: &["ns4", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};
