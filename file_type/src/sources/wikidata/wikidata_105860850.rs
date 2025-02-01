use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860850: FileFormat = FileFormat {
    id: 105_860_850,
    puid: "wikidata/105860850",
    name: "SQL Server Reporting Services Report Definition Language",
    extensions: &["rdl", "rdlc"],
    media_types: &["text/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
