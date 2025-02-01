use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263338: FileFormat = FileFormat {
    id: 111_263_338,
    puid: "wikidata/111263338",
    name: "DirectMusic Producer DLS file",
    extensions: &["dlp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
