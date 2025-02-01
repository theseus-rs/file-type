use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131232034: FileFormat = FileFormat {
    id: 131_232_034,
    puid: "wikidata/131232034",
    name: "Allotrope Data Format",
    extensions: &["adf"],
    media_types: &["application/x-hdf5"],
    internal_signatures: &[],
    related_formats: &[],
};
