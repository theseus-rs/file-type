use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_46120375: FileFormat = FileFormat {
    id: 46_120_375,
    puid: "wikidata/46120375",
    name: "Lotus Notes Database file format, version 4",
    extensions: &["ns4", "nsf"],
    media_types: &["application/vnd.lotus-notes", "application/vnd.lotus-notes"],
    internal_signatures: &[],
    related_formats: &[],
};
