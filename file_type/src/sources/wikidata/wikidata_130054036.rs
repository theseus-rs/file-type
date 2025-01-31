use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130054036: FileFormat = FileFormat {
    id: 130_054_036,
    puid: "wikidata/130054036",
    name: "Juttle source code file",
    extensions: &["juttle", "juttle", "juttle", "juttle"],
    media_types: &[
        "application/juttle",
        "application/x-juttle",
        "text/juttle",
        "text/x-juttle",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
