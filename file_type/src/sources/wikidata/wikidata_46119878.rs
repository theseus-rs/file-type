use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_46119878: FileFormat = FileFormat {
    id: 46_119_878,
    source_type: SourceType::Wikidata,
    name: "Lotus Notes Database file format, version 2",
    extensions: &["ns2", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
