use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_46119878: FileFormat = FileFormat {
    id: 46_119_878,
    source_type: SourceType::Wikidata,
    name: "Lotus Notes Database file format, version 2",
    extensions: &["ns2", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};
