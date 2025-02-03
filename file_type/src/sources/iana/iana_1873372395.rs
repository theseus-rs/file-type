use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1873372395: FileFormat = FileFormat {
    id: 1_873_372_395,
    source_type: SourceType::Iana,
    name: "vnd.ibm.afplinedata (OBSOLETED in favor of vnd.afpc.afplinedata)",
    extensions: &[],
    media_types: &["application/vnd.ibm.afplinedata"],
    internal_signatures: &[],
    related_formats: &[],
};
