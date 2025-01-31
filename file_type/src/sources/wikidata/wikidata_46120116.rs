use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_46120116: FileFormat = FileFormat {
    id: 46_120_116,
    puid: "wikidata/46120116",
    name: "Lotus Notes Database file format, version 3",
    extensions: &["ns3", "nsf"],
    media_types: &["application/vnd.lotus-notes", "application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};
