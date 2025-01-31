use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837148: FileFormat = FileFormat {
    id: 108_837_148,
    puid: "wikidata/108837148",
    name: "Quicken Data Backup",
    extensions: &["qdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
