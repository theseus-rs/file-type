use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771322: FileFormat = FileFormat {
    id: 28_771_322,
    puid: "wikidata/28771322",
    name: "Microsoft Works Database file format (backup file)",
    extensions: &["bdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
