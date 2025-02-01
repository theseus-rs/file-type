use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344823: FileFormat = FileFormat {
    id: 28_344_823,
    puid: "wikidata/28344823",
    name: "Debian source control file",
    extensions: &["dsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
