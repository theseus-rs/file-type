use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131294117: FileFormat = FileFormat {
    id: 131_294_117,
    puid: "wikidata/131294117",
    name: "Terraform file format",
    extensions: &["hcl", "hcl", "tf", "tf"],
    media_types: &[
        "application/x-terraform",
        "application/x-terraform",
        "application/x-tf",
        "application/x-tf",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
