use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1546911: FileFormat = FileFormat {
    id: 1_546_911,
    puid: "wikidata/1546911",
    name: "Cross-Platform Installer Module",
    extensions: &["xpi"],
    media_types: &["application/x-xpinstall"],
    internal_signatures: &[],
    related_formats: &[],
};
