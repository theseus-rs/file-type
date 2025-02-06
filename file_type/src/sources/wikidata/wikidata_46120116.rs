use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_46120116: FileFormat = FileFormat {
    id: 46_120_116,
    source_type: SourceType::Wikidata,
    name: "Lotus Notes Database file format, version 3",
    extensions: &["ns3", "nsf"],
    media_types: &["application/vnd.lotus-notes"],
    signatures: &[],
    related_formats: &[],
};
