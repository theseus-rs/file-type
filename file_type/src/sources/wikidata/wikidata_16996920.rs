use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_16996920: FileFormat = FileFormat {
    id: 16_996_920,
    puid: "wikidata/16996920",
    name: "Windows Setup Information File",
    extensions: &["inf", "inf", "inf", "inf"],
    media_types: &[
        "application/inf",
        "application/x-setupscript",
        "application/x-wine-extension-inf",
        "text/x-inf",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
