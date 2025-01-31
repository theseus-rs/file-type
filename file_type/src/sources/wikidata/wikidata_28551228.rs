use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551228: FileFormat = FileFormat {
    id: 28_551_228,
    puid: "wikidata/28551228",
    name: "Adobe Action File",
    extensions: &["atn"],
    media_types: &["application/x-photoshop"],
    internal_signatures: &[],
    related_formats: &[],
};
