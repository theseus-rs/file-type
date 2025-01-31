use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205410: FileFormat = FileFormat {
    id: 28_205_410,
    puid: "wikidata/28205410",
    name: "Nikon Dust File",
    extensions: &["ndf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
