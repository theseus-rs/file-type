use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28791369: FileFormat = FileFormat {
    id: 28_791_369,
    puid: "wikidata/28791369",
    name: "App Installer package",
    extensions: &["appx", "appxbundle"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
