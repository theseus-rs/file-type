use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128597273: FileFormat = FileFormat {
    id: 128_597_273,
    puid: "wikidata/128597273",
    name: "Ampl file",
    extensions: &["run"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
